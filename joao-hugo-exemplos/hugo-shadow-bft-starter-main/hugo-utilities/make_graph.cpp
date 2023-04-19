#include <bits/stdc++.h>
using namespace std;

void make_graph(int nodes){
  cout << "graph [" << '\n' << "  directed 1" << '\n';

  for(int i=0; i<nodes; ++i){
    cout << "  node [" << '\n';
    cout << "    id " << i << '\n';
    cout << "    host_bandwidth_up   \"" << rand()%10000 + 1 << " Mibit\"" << '\n';
    cout << "    host_bandwidth_down \"" << rand()%10000 + 1 << " Mibit\"" << '\n';
    cout << "  ] " << '\n';
  }

  for(int i=0; i<nodes; ++i){
    for(int j=0; j<nodes; ++j){
      cout << "  edge [" << '\n';
      cout << "    source " << i << '\n';
      cout << "    target " << j << '\n';
      cout << "    latency \"" << rand()%10000 << " us\"" << '\n';
      int num = rand()%1000 + 1;
      int denom = rand()%1500 + num;
      double val = (double) num / (double) (denom);
      cout << "    packet_loss " << val << '\n';
      cout << "  ] " << '\n';
    }
  }
  cout << "] \n";
}


signed main(int argc, char **argv){
  srand(time(NULL));

  if(argc != 2){
    cout << "Make graph requires the number of nodes as the only argument\n";
  }
  int nodes = stoi(argv[1]);

  make_graph(nodes);
}