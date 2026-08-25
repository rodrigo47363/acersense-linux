"""
AcerSense Background Daemon
Runs in background to enforce 80% battery limits, fan profiles, and restore settings across sleep/wake cycles.
"""

import time
import signal
import sys
import logging
from acersense.core.profile_manager import ProfileManager
from acersense.hardware.battery_interface import BatteryInterface
from acersense.hardware.thermal_interface import ThermalInterface

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(name)s: %(message)s")
logger = logging.getLogger("acersensed")


class AcerSenseDaemon:
    def __init__(self):
        self.profile_mgr = ProfileManager()
        self.battery = BatteryInterface(self.profile_mgr.wmi)
        self.thermal = ThermalInterface()
        self._running = True

        signal.signal(signal.SIGINT, self._handle_signal)
        signal.signal(signal.SIGTERM, self._handle_signal)

    def _handle_signal(self, signum, frame):
        logger.info("Termination signal received. Shutting down daemon...")
        self._running = False

    def run(self):
        logger.info("Starting AcerSense Hardware Daemon...")
        
        # Apply saved profile on daemon startup
        saved_profile = self.profile_mgr.config.get("active_profile", "balanced")
        logger.info(f"Applying startup profile: {saved_profile}")
        try:
            self.profile_mgr.apply_profile(saved_profile)
        except Exception as e:
            logger.error(f"Error applying startup profile: {e}")

        while self._running:
            try:
                # 1. Enforce battery 80% limit if configured
                bat_cfg = self.profile_mgr.config.get("battery", {})
                if bat_cfg.get("health_mode_80_limit", True):
                    cur_limit = self.battery.wmi.get_battery_health_mode()
                    if cur_limit is False:
                        logger.info("Enforcing 80% battery health limit...")
                        self.battery.set_80_percent_limit(True)

                # Sleep interval
                time.sleep(10.0)
            except Exception as e:
                logger.error(f"Error in daemon loop: {e}")
                time.sleep(5.0)

        logger.info("AcerSense Daemon terminated cleanly.")


def main():
    daemon = AcerSenseDaemon()
    daemon.run()


if __name__ == "__main__":
    main()
