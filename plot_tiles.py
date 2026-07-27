import matplotlib.pyplot as plt
import matplotlib.patches as patches
import json
import tkinter as tk
from tkinter import filedialog


root = tk.Tk()
root.withdraw()

path = filedialog.askopenfilename()

try:
    with open(path, 'r', encoding='utf-8-sig') as f:
        data = json.load(f)
except UnicodeError:
    with open(path, 'r', encoding='utf-16') as f:
        data = json.load(f)

fig, ax = plt.subplots(figsize=(8, 8))

for i, tile in enumerate(data["Tile"]):
    x1, y1 = tile["Left Corner"]
    x2, y2 = tile["Right Corner"]
    
    x_min, x_max = min(x1, x2), max(x1, x2)
    y_min, y_max = min(y1, y2), max(y1, y2)
    width = x_max - x_min
    height = y_max - y_min
    
    rect = patches.Rectangle(
        (x_min, y_min), width, height, 
        linewidth=1.5, edgecolor='blue', facecolor='cyan', alpha=0.4
    )
    ax.add_patch(rect)
    
    cx = x_min + (width / 2) if width > 0 else x_min
    cy = y_min + (height / 2) if height > 0 else y_min
    ax.text(cx, cy, str(i+1), color='black', ha='center', va='center', fontsize=10, fontweight='bold')


    if "Room" in tile:
        rx1, ry1 = tile["Room"]["Left Corner"]
        rx2, ry2 = tile["Room"]["Right Corner"]
        
        rx_min, rx_max = min(rx1, rx2), max(rx1, rx2)
        ry_min, ry_max = min(ry1, ry2), max(ry1, ry2)
        r_width = rx_max - rx_min
        r_height = ry_max - ry_min
        
        # Plotted the room as a darker, more opaque red
        room_rect = patches.Rectangle(
            (rx_min, ry_min), r_width, r_height, 
            linewidth=1.5, edgecolor='indigo', facecolor='blue', alpha=0.6
        )
        ax.add_patch(room_rect)
    

ax.set_xlim(0, 2120)
ax.set_ylim(0, 2120)
ax.invert_yaxis() 
ax.set_aspect('equal')
ax.set_title("Generated BSP Map Tiles")
ax.set_xlabel("X Coordinate")
ax.set_ylabel("Y Coordinate")
plt.grid(True, linestyle='--', alpha=0.6)

plt.show()
