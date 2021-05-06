pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/contracts/utils/Counters.sol";

contract Poke is ERC721
{
    
    enum Pokcolor{ BLUE, GREEN, RED, YELLOW }
    
    uint8 constant StandardMin = 1;
    uint8 constant StandardMax = 3;
    uint8 constant BoostedMin = 3;
    uint8 constant BoostedMax = 5;
    
    struct Mecha {
        uint8 Atk;
        uint8 Health;
        uint8 Speed;
        Pokcolor Color; 
        string Name;
    }

    
    uint randNonce;
    uint public randomState;
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    
    mapping(uint256 => Mecha) public MechasOwnership;
    
    constructor () ERC721("Poke", "PKT") {
            

        
    }
    
    function mint(string calldata name) public {
            
        _tokenIds.increment();
        uint newItemId = _tokenIds.current();
        
        uint LIntColor = randMod(uint(Pokcolor.YELLOW) + 1);
        
        Mecha memory newMecha;
        newMecha.Color = Pokcolor(LIntColor);
        newMecha.Name = name;
        if (newMecha.Color == Pokcolor.BLUE) 
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomBoostedStat();
            newMecha.Speed = GetRandomStandardStat();
            
        }
        else if (newMecha.Color == Pokcolor.GREEN)
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomStandardStat();
            newMecha.Speed = GetRandomStandardStat();
        }
        else if (newMecha.Color == Pokcolor.RED)
        {
            newMecha.Atk = GetRandomBoostedStat();
            newMecha.Health = GetRandomStandardStat();
            newMecha.Speed = GetRandomStandardStat();
        }
        else if (newMecha.Color == Pokcolor.YELLOW)
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomStandardStat(); 
            newMecha.Speed = GetRandomBoostedStat();
        }
        
        
        _safeMint(msg.sender, newItemId);

        MechasOwnership[newItemId] = newMecha;
        
    }
    
    // not very goog RNG
    function randMod(uint _modulus) private returns(uint) 
    {
       // increase nonce
       randNonce++;  
       return uint(keccak256(abi.encodePacked(block.timestamp, 
                                              msg.sender, 
                                              randNonce))) % _modulus;
    }
    
    function GetRandomStandardStat() private returns(uint8)
    {
        return uint8(randMod(StandardMax) + 1);
    }
    
    function GetRandomBoostedStat() private returns(uint8)
    {
        return uint8(randMod(StandardMax) + 3);
    }
    
}