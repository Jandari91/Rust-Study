/*

## Basic Memory
메모리 주소와 오프셋 등을 포함한 메모리에 대한 중급 개념들입니다.

메모리는 0과 1로만 표현되는 비트를 통해 이진으로 저장되며 여기서 비트는 메모리에 저장 될 수 있는 가장 작은 단위입니다.
컴퓨터 하드웨어는 바이트로 작업하는 데 최적화 되어 있습니다.

1 바이트는 연솓 된 8 개의 비트와 같으며 ㄸ라ㅏ서 컴퓨터 전체 메모리는 비트들로 이루어진 하나의 연속된 끈으로 볼 수 있습니다.


## Addresses
모든 데이터 메모리는 메모리 주소라는 것을 갖고 있는데, 이 주소는 메모리에서의 데이터의 위치를 특정하는 데 쓰입니다.
주소는 항상 같습니다. 단지 그 주소의 데이터만 바뀔 뿐입니다.
보통은 프로그램을 만들 때 이 주소들을 직접 쓰는 대신 변수를 이용합니다.

## Offsets

메모리 오프셋은 특정 주소에 있는 항목의 위치를 특정하는 데 사용됩니다.
오프셋은 항상 0에서 시작하며 원래의 주소로 부터 몇 바이트 떨어져 있는지 나타냅니다.
주소와 마찬가지로 보통 오프셋을 직접 사용하지는 않습니다.
그 대신 인덱스를 사용하면 컴파일러가 인덱스에 기반해 원래 주소로부터 얼마나 떨어져 있는지 자동으로 계산합니다.


아래의 돞표는 컴퓨터 메모리에서의 주소와 오프셋을 보여줍니다.

* 세로는 Address고 가로는 Offset입니다.
* □ = 1 byte 입니다.
* ■ 는 Address 4, offset1, Data[1] 입니다.
||0|1|2|3|
|-|-|-|-|-|
|0|□|□|□|□|
|4|□|■|□|□|
|8|□|□|□|□|
|12|□|□|□|□|
|16|□|□|□|□|

Data 라는 변수는 세로의 주소를 나타내고 [1]은 가로인 Offset을 나타냅니다.

메모리는 주소와 오프셋을 사용하며 주소는 영구적이지만 주소에 있는 데이터는 바뀝니다.
오프셋으로 어떤 데이터에 인덱싱 할 수 있습니다.
따라서 어떤 항목들이 있는 리스트가 있다면 인덱스를 이용해 리스트 안의 각각의 항목에 접근 할 수 있게 됩니다.

*/