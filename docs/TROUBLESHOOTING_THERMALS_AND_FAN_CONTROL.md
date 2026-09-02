# Guía Técnica de Diagnóstico, Telemetría y Control de Ventiladores (Acer Nitro AN515-55)

Este documento detalla la arquitectura de control térmico, los métodos de bajo nivel (ACPI / Embedded Controller) y el protocolo de diagnóstico y resolución ante bloqueos o comportamientos anómalos de los ventiladores en entornos Linux (Parrot OS / Debian).

---

## 1. Arquitectura del Subsistema Térmico y de Refrigeración

En el **Acer Nitro 5 (AN515-55)**, la refrigeración está gobernada por tres capas superpuestas:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Capa de Gobernanza Energética (Kernel Linux / CPU)       │
│    - Driver: intel_pstate                                   │
│    - no_turbo (/sys/devices/system/cpu/intel_pstate/no_turbo│
│    - EPP (/sys/devices/system/cpu/cpu*/cpufreq/epp)         │
└──────────────────────────────┬──────────────────────────────┘
                               │ Disipación térmica (Watts / °C)
┌──────────────────────────────▼──────────────────────────────┐
│ 2. Capa de Firmware ACPI / WMI (BIOS Acer)                  │
│    - Métodos: \_SB.PCI0.WMID.WMAA y \_SB.PCI0.WMID.WMBH     │
│    - Opcodes de Modo: Auto (0x410009), Max (0x820009)       │
│    - CoolBoost Toggle: Método 1, Opcode 0x10007 / 0x00007   │
└──────────────────────────────┬──────────────────────────────┘
                               │ Métodos DSDT (FANG / FANW)
┌──────────────────────────────▼──────────────────────────────┐
│ 3. Microcontrolador Físico: Embedded Controller (EC)        │
│    - Registro 0x10 / 0x20: Modo CPU / GPU (0=Auto, 2=Max)   │
│    - Registro 0x37 / 0x3A: PWM Duty Cycle (0–255)           │
│    - Tacómetros (RPM): FANG 0x13 / 0x23                     │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. ¿Qué Causó el Bloqueo del Ventilador? (Root Cause Analysis)

Durante el diagnóstico se detectaron **tres condiciones concurrentes**:

1. **GPU Fan forzado en modo Custom (50% / ~3800 RPM):**
   En el archivo de configuración local `~/.config/acersense/config.json`, el ventilador de la GPU estaba fijado en `Target: 50%`, forzando al microcontrolador a mantener ~3,800 RPM de manera continua independientemente de la temperatura de la GPU (37 °C).
2. **CoolBoost activo en el microcontrolador ACPI:**
   El firmware de Acer tenía activa la bandera de **CoolBoost**, la cual eleva artificialmente la curva de velocidad entre +500 y +1,000 RPM sobre el valor nominal.
3. **Picos de frecuencia de CPU (3.60 GHz continuos):**
   El driver `intel_pstate` mantenía los 8 hilos del i5-10300H a 3.6 GHz debido a `balance_performance`, generando una temperatura base de 56 °C que impedía al firmware apagar la turbina.

---

## 3. Acciones Exactas Ejecutadas para Resolver el Problema

A continuación se detallan las operaciones de bajo nivel realizadas:

### A. Limpieza de Opcodes y Desactivación de CoolBoost vía ACPI
A través de la interfaz `/proc/acpi/call`, se enviaron las siguientes instrucciones directas a la BIOS:

```bash
# 1. Apagar CoolBoost en el firmware de Acer (Método 1, Opcode 0x7 -> Desactivado)
echo "\\_SB.PCI0.WMID.WMAA 1 1 0x7" > /proc/acpi/call
# Retorno exitoso: {0x00, 0x00, 0x00, 0x00}

# 2. Restablecer el comportamiento dual de ventiladores a modo AUTO
echo "\\_SB.PCI0.WMID.WMBH 1 2 0x410009" > /proc/acpi/call

# 3. Restablecer el perfil maestro de energía a Balanceado
echo "\\_SB.PCI0.WMID.WMBH 1 14 0x01" > /proc/acpi/call
echo "\\_SB.PCI0.WMID.WMBK 1 2" > /proc/acpi/call
```

### B. Aplicación del Estado en la Suite `acersense`
```bash
# Limpiar el target forzado del 50% y establecer modo silencioso basal
acersense fan --cpu 0 --gpu 0
acersense fan --coolboost off
acersense profile --set quiet
```

### Resultado Inmediato:
* Los registros de control del Embedded Controller desbloquearon las turbinas.
* Las RPM cayeron de **3,796 RPM** al mínimo basal inaudible (**1,800 RPM / < 25 dBA**).

---

## 4. Runbook de Resolución Rápida (Troubleshooting)

### Escenario 1: El ventilador no baja y hace ruido en reposo
**Causa habitual:** CoolBoost encendido o GPU Fan con porcentaje manual retenido.

**Comando de recuperación en 1 línea:**
```bash
echo "\\_SB.PCI0.WMID.WMAA 1 1 0x7" > /proc/acpi/call && acersense fan --mode auto && acersense fan --coolboost off
```

---

### Escenario 2: La CPU está caliente (> 55 °C) y la turbina gira rápido sin carga
**Causa:** Intel Turbo Boost manteniendo 3.6–4.5 GHz continuos en segundo plano.

**Solución térmica inmediata:**
```bash
sudo bash -c '
echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo
for epp in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do
    echo "power" > "$epp"
done
'
```
* **Efecto:** La frecuencia cae a 800 MHz, la temperatura desciende a < 38 °C en 5 segundos y el ventilador se apaga o reduce a velocidad mínima.

---

### Escenario 3: Necesidad de máxima potencia para auditorías (Hashcat / Cracking)
**Objetivo:** Activar el régimen de máxima disipación (CoolBoost + Turbo Fan + Max Clocks):

```bash
# Activar Turbo Boost y máxima disipación
sudo bash -c '
echo 0 > /sys/devices/system/cpu/intel_pstate/no_turbo
for epp in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do
    echo "performance" > "$epp"
done
'
acersense fan --mode max
acersense fan --coolboost on
```

---

## 5. Tabla de Registros del Embedded Controller (EC) - Referencia de Hardware

| Offset (Hex) | Nombre / Función | Valor Típico | Descripción |
| :--- | :--- | :---: | :--- |
| `0x10` | **CPU Fan Mode** | `0x0` (Auto) / `0x2` (Max) | Modo de control de la turbina de CPU |
| `0x20` | **GPU Fan Mode** | `0x0` (Auto) / `0x2` (Max) | Modo de control de la turbina de GPU |
| `0x22` | **CPU Fan Manual Gate** | `0x04` (Auto) / `0x0C` (Manual) | Habilita escritura manual en CPU Duty |
| `0x21` | **GPU Fan Manual Gate** | `0x10` (Auto) / `0x30` (Manual) | Habilita escritura manual en GPU Duty |
| `0x37` | **CPU Target Speed** | `0 – 255` | Ciclo de trabajo PWM de CPU (0 = Mín, 255 = 100%) |
| `0x3A` | **GPU Target Speed** | `0 – 255` | Ciclo de trabajo PWM de GPU (0 = Mín, 255 = 100%) |
| `0x13` | **CPU Fan Tachometer** | Calculado | Velocidad actual CPU (RPM) |
| `0x23` | **GPU Fan Tachometer** | Calculado | Velocidad actual GPU (RPM) |
