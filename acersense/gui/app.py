"""
AcerSense GUI Launcher
Launches the Next-Gen Cyberpunk Hardware Control Center via native app window or CustomTkinter fallback.
"""

import sys
import os
import shutil
import subprocess
import threading
import time
import webbrowser
from typing import Optional

from acersense.gui.web.server import run_server


def launch_web_ui(port: int = 18888):
    """Starts local backend server and launches standalone Chromium/Web window."""
    # 1. Start HTTP/REST Server in daemon thread
    server = run_server(port)
    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()

    url = f"http://127.0.0.1:{port}"
    user_data_dir = os.path.expanduser("~/.config/acersense/browser_profile")
    os.makedirs(user_data_dir, exist_ok=True)

    # Common security/isolation flags to avoid loading obsolete system distro extensions
    chrome_flags = [
        f"--app={url}",
        f"--user-data-dir={user_data_dir}",
        "--window-size=1080,780",
        "--class=acersense-gui",
        "--disable-extensions",
        "--disable-default-apps",
        "--disable-component-extensions-with-background-pages",
        "--no-first-run",
        "--no-default-browser-check",
        "--disable-background-networking",
        "--disable-sync",
        "--disable-translate",
        "--hide-scrollbars=false"
    ]

    # 2. Check for Chromium / Google Chrome / Brave for standalone App Window mode
    browsers = [
        ("chromium", ["chromium"] + chrome_flags),
        ("google-chrome", ["google-chrome"] + chrome_flags),
        ("brave-browser", ["brave-browser"] + chrome_flags),
        ("firefox", ["firefox", "--new-window", url])
    ]

    launched = False
    for name, cmd in browsers:
        if shutil.which(name):
            try:
                # Suppress distro extension warnings in terminal output
                proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                launched = True
                proc.wait()
                break
            except Exception:
                pass

    if not launched:
        # Fallback to default browser
        webbrowser.open(url)
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            pass


def main():
    if "--native" in sys.argv or "--ctk" in sys.argv:
        # Fallback to pure Tkinter UI if user explicitly requests native widgets
        import tkinter as tk
        try:
            import customtkinter as ctk
            root = ctk.CTk()
        except ImportError:
            root = tk.Tk()
        from acersense.gui.app_ctk import AcerSenseGUI
        app = AcerSenseGUI(root)
        root.mainloop()
    else:
        launch_web_ui()


if __name__ == "__main__":
    main()
