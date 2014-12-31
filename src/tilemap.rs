use sprite::Sprite;
use std::vec::Vec;
use render::{Render, Draw};

enum Tiles {
    TileUnused,
    TileBrickFloor,
    TileBrickRoomFloor,
    TileStoneWall,
    TileMetalFloor,
    TileCorridor,
    TileDoor,
    TileUpStairs,
    TileDownStairs,
    TileChest
}

pub struct TileMap {
    pub tileset: Sprite,
    pub map: Vec<uint>,
    pub tiles: Vec<Sprite>,

    iXSize: uint,
    iYSize: uint,
    iXMax: uint,
    iYMax:uint,
    iObjects:uint,
    iChanceRoom:uint,
    iChanceCorridor:uint,
}

impl TileMap {
    pub fn from_tileset_path(path: &Path) -> TileMap {
        TileMap {
            // Initialize the tileset
            tileset: Sprite::from_path(path),

            // Initialize the tiles
            tiles: Vec::new(),

            // Initialize common values
            iChanceRoom: 75u,
            iChanceCorridor: 25u,
            iXSize: 0u,
            iYSize: 0u,
            iXMax: 30u,
            iYMax: 30u,
            iObjects: 10u,

            // Initialize the vector of map
            map: Vec::new(),
            //enemies_map: Vec::new();
            //items_map: Vec::new();
        }
    }

    //pub fn build_tiles_from_tileset(&self) {
      //  self.tiles.push();

    //}

    pub fn build_procedural_map(&mut self, width: uint, height: uint, dungeonObjects: uint) -> Vec<uint>
    {
        // Create the map
        if (dungeonObjects < 1u) {
            self.iObjects = 10u;
        }
        else {
            self.iObjects = dungeonObjects;
        }

        // We need to ensure a max map size wont be overlaped
        if (width > self.iXMax) {
            self.iXSize = self.iXMax;
        }
        else {
            self.iXSize = width;
        }

        if (height > self.iYMax) {
            self.iYSize = self.iYMax;
        }
        else {
            self.iYSize = height;
        }

        // Create the borders and fill the midle with dirt
        for y in range(0u, self.iYSize) {
            for x in range(0u, self.iXSize) {
                //Making the borders of unwalkable walls
                if (y == 0u) {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if (y == self.iYSize - 1) {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if (x == 0u) {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if (x == self.iXSize - 1u) {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                //Fill the rest with dirt
                else {
                    //self.set_tile(x, y, Tiles::TileBrickFloor as uint);
                    self.map.push(Tiles::TileBrickFloor as uint);
                }

                // Initialize the enemies
                //this->SetEnemy(x, y, enemyNull);

                // Initialize the items
                //this->SetItem(x, y, itemNull);
            }
        }

        // Ugly, try to return a reference instead
        self.map.clone()

/*
        // Create the ramdom rooms
        // Start with making a room in the middle
        this->MakeRoom(iXSize / 2, iYSize / 2, 8, 6, clRand.Get(0, 3));

        // Keep count of the number of "objects" we've made
        int currentFeatures = 1; //+1 for the first room we just made

        // Then we sart the main loop to build the world
        for (s32 countingTries = 0; countingTries < 1000; countingTries++)
        {
            //check if we've reached our quota
            if (currentFeatures == iObjects)
                break;

            //start with a random wall
            s32 newx = 0;
            s32 xmod = 0;
            s32 newy = 0;
            s32 ymod = 0;
            s32 validTile = -1;
            //1000 chances to find a suitable object (room or corridor)..
            //(yea, i know it's kinda ugly with a for-loop... -_-')
            for (int testing = 0; testing < 1000; testing++)
            {
                newx = clRand.Get(1, iXSize - 1);
                newy = clRand.Get(1, iYSize - 1);
                validTile = -1;
                if (this->GetTile(newx, newy) == tileStoneWall)
                {
                    //check if we can reach the place
                    if (this->GetTile(newx, newy + 1) == tileBrickRoomFloor)
                    {
                        validTile = 0; //
                        xmod = 0;
                        ymod = -1;// BUG HERE.. int??
                    }
                    else if (this->GetTile(newx - 1, newy) == tileBrickRoomFloor)
                    {
                        validTile = 1; //
                        xmod = +1;
                        ymod = 0;
                    }
                    else if (this->GetTile(newx, newy - 1) == tileBrickRoomFloor)
                    {
                        validTile = 2; //
                        xmod = 0;
                        ymod = +1;
                    }
                    else if (this->GetTile(newx + 1, newy) == tileBrickRoomFloor)
                    {
                        validTile = 3; //
                        xmod = -1;//BUG HERE.. int??
                        ymod = 0;
                    }

                    //check that we haven't got another door nearby, so we won't get alot of openings besides
                    //each other
                    if (validTile > -1)
                    {
                        if (this->GetTile(newx, newy + 1) == tileDoor) //north
                            validTile = -1;
                        else if (this->GetTile(newx - 1, newy) == tileDoor)//east
                            validTile = -1;
                        else if (this->GetTile(newx, newy - 1) == tileDoor)//south
                            validTile = -1;
                        else if (this->GetTile(newx + 1, newy) == tileDoor)//west
                            validTile = -1;
                    }

                    //if we can, jump out of the loop and continue with the rest
                    if (validTile > -1)
                        break;
                }
            }

            if (validTile > -1)
            {
                //choose what to build now at our newly found place, and at what direction
                int feature = clRand.Get(0, 100);
                if (feature <= iChanceRoom) //a new room
                {
                    if (this->MakeRoom((newx + xmod), (newy + ymod), 8, 6, validTile))
                    {
                        currentFeatures++; //add to our quota

                        //then we mark the wall opening with a door
                        this->SetTile(newx, newy, tileDoor);

                        //clean up infront of the door so we can reach it
                        this->SetTile((newx + xmod), (newy + ymod), tileBrickRoomFloor);
                    }
                }
                else if (feature >= iChanceRoom) //new corridor
                {
                    if (this->MakeCorridor((newx + xmod), (newy + ymod), 6, validTile))
                    {
                        //same thing here, add to the quota and a door
                        currentFeatures++;

                        this->SetTile(newx, newy, tileDoor);
                    }
                }
            }
        }*/
    }

    pub fn set_tile(&mut self, x: uint, y: uint, tile_type: uint) {
        self.map[x + self.iXSize * y] = tile_type;
    }

}

impl Draw for TileMap {
    fn draw(&self, render: &mut Render) {
        //println!("Size of image: {}", &self.tileset.image.);
        render.draw(&self.tileset);
    }
}

