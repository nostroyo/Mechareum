pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/contracts/utils/Counters.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Enumerable.sol";

contract Mecha is ERC721Enumerable
{

    enum MechaColor{ RED, BLUE, GREEN, YELLOW }

    uint8 constant StandardMin = 1;
    uint8 constant StandardMax = 3;
    uint8 constant BoostedMin = 3;
    uint8 constant BoostedMax = 5;

    struct Mecha {
        uint8 Atk;
        uint8 Health;
        uint8 Speed;
        MechaColor Color;
        string Name;
    }


    uint randNonce;
    uint public randomState;
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    mapping(uint256 => Mecha) private idToMechaMap;


    constructor () ERC721("Poke", "PKT") {
    }

    function mint(string calldata name) public {

        _tokenIds.increment();
        uint newItemId = _tokenIds.current();

        uint LIntColor = randMod(uint(MechaColor.YELLOW) + 1);

        Mecha memory newMecha;
        newMecha.Color = MechaColor(LIntColor);
        newMecha.Name = name;
        if (newMecha.Color == MechaColor.BLUE)
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomBoostedStat();
            newMecha.Speed = GetRandomStandardStat();

        }
        else if (newMecha.Color == MechaColor.GREEN)
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomStandardStat();
            newMecha.Speed = GetRandomStandardStat();
        }
        else if (newMecha.Color == MechaColor.RED)
        {
            newMecha.Atk = GetRandomBoostedStat();
            newMecha.Health = GetRandomStandardStat();
            newMecha.Speed = GetRandomStandardStat();
        }
        else if (newMecha.Color == MechaColor.YELLOW)
        {
            newMecha.Atk = GetRandomStandardStat();
            newMecha.Health = GetRandomStandardStat();
            newMecha.Speed = GetRandomBoostedStat();
        }


        _safeMint(msg.sender, newItemId);

        idToMechaMap[newItemId] = newMecha;

    }

    function GetMechaById(uint256 id) public view returns(Mecha memory) {
        require(msg.sender == ownerOf(id));

        return idToMechaMap[id];
    }

    // not very good RNG use oracle instead
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