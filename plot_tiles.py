import json
import matplotlib.pyplot as plt
import tkinter as tk
from tkinter import filedialog

def draw_wireframe_box(ax, lc, rc, color='blue', linewidth=1.0, alpha=1.0):
    """Draws a 3D bounding box using line segments."""
    x = [lc[0], rc[0]]
    y = [lc[1], rc[1]]
    z = [lc[2], rc[2]]

    # Draw the bottom face (Z = z[0])
    ax.plot([x[0], x[1], x[1], x[0], x[0]], 
            [y[0], y[0], y[1], y[1], y[0]], 
            [z[0], z[0], z[0], z[0], z[0]], 
            color=color, lw=linewidth, alpha=alpha)
    
    # Draw the top face (Z = z[1])
    ax.plot([x[0], x[1], x[1], x[0], x[0]], 
            [y[0], y[0], y[1], y[1], y[0]], 
            [z[1], z[1], z[1], z[1], z[1]], 
            color=color, lw=linewidth, alpha=alpha)
    
    # Draw the four vertical pillars connecting top and bottom
    for i in range(2):
        for j in range(2):
            ax.plot([x[i], x[i]], [y[j], y[j]], [z[0], z[1]], 
                    color=color, lw=linewidth, alpha=alpha)

def main():

    root = tk.Tk()
    root.withdraw()
    path = filedialog.askopenfilename()

    try:
        with open(path, 'r', encoding='utf-16') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        with open(path, 'r') as f:
            data = json.load(f)
    except FileNotFoundError:
        print("Error: json not found")
        return

    # Setup the 3D plot
    fig = plt.figure(figsize=(10, 8))
    ax = fig.add_subplot(111, projection='3d')

    # Iterate through the tiles and plot
    for tile in data.get('Tiles', []):
        # 1. Plot the BSP Partition Tile (Faint Gray)
        draw_wireframe_box(
            ax, 
            tile['Left Corner'], 
            tile['Right Corner'], 
            color='gray', 
            linewidth=0.5, 
            alpha=0.3
        )
        
        # 2. Plot the Room if it exists (Bold Blue)
        if tile.get('Room') is not None:
            room = tile['Room']
            draw_wireframe_box(
                ax, 
                room['Left Corner'], 
                room['Right Corner'], 
                color='blue', 
                linewidth=2.0
            )

    # Format the axes
    ax.set_xlabel('X Axis')
    ax.set_ylabel('Y Axis')
    ax.set_zlabel('Z Axis')
    ax.set_title('3D BSP Dungeon Generation')

    # Force the aspect ratio to be equal so rooms aren't distorted
    ax.set_box_aspect([1, 1, 1]) 

    plt.show()

if __name__ == "__main__":
    main()