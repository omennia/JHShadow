#include <bits/stdc++.h>
#include <fstream>
using namespace std;

/* Default variables - For now global*/

ofstream mf;


class systema{
    public:
    string shutdownhook = "true";
    string samebatchsize = "false";
    string bft = "true";
    long long numrepliers = 16;
    
    void print_c_name(){
        mf << "system.";
    }

    void print(){
        print_c_name(), mf << "shutdownhook = " << shutdownhook << '\n';
        print_c_name(), mf << "samebatchsize = " << samebatchsize << '\n';
        print_c_name(), mf << "numrepliers = " << numrepliers << '\n';
        print_c_name(), mf << "bft = " << bft << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "shutdownhook") this->shutdownhook = param;
        if(obs == "samebatchsize") this->samebatchsize = param;
        if(obs == "bft") this->bft = param;
        if(obs == "numrepliers") this->numrepliers = stoi(param);
    }
};

class communication : public systema{
    public:

    string secretKeyAlgorithm = "PBKDF2WithHmacSHA1", secretKeyAlgorithmProvider = "SunJCE";
    string hashAlgorithm = "SHA-256", hashAlgorithmProvider = "SUN";
    string signatureAlgorithm = "SHA256withECDSA", signatureAlgorithmProvider = "BC";
    string defaultKeyLoader = "ECDSA", useSenderThread = "true";
    string defaultkeys = "true", bindaddress = "auto";
    long long inQueueSize = 500000;
    long long outQueueSize = 500000;
    long long useSignatures = 0;
    
    void print_sc_name(){
        print_c_name(), mf << "communication.";
    }

    void print(){
        print_sc_name(), mf << "secretKeyAlgorithm = " << secretKeyAlgorithm << '\n';
        print_sc_name(), mf << "secretKeyAlgorithmProvider = " << secretKeyAlgorithmProvider << '\n';
        print_sc_name(), mf << "hashAlgorithm = " << hashAlgorithm << '\n';
        print_sc_name(), mf << "hashAlgorithmProvider = " << hashAlgorithmProvider << '\n';
        print_sc_name(), mf << "signatureAlgorithm = " << signatureAlgorithm << '\n';
        print_sc_name(), mf << "signatureAlgorithmProvider = " << signatureAlgorithmProvider << '\n';
        print_sc_name(), mf << "defaultKeyLoader = " << defaultKeyLoader << '\n';
        print_sc_name(), mf << "useSenderThread = " << useSenderThread << '\n';
        print_sc_name(), mf << "defaultkeys = " << defaultkeys << '\n';
        print_sc_name(), mf << "bindaddress = " << bindaddress << '\n';
        print_sc_name(), mf << "inQueueSize = " << inQueueSize << '\n';
        print_sc_name(), mf << "outQueueSize = " << outQueueSize << '\n';
        print_sc_name(), mf << "useSignatures = " << useSignatures << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "secretKeyAlgorithm") this->secretKeyAlgorithm = param;
        if(obs == "hashAlgorithm") this->hashAlgorithm = param;
        if(obs == "signatureAlgorithm") this->signatureAlgorithm = param;
        if(obs == "defaultKeyLoader") this->defaultKeyLoader = param;
        if(obs == "defaultkeys") this->defaultkeys = param;
        if(obs == "inQueueSize") this->inQueueSize = stoi(param);
        if(obs == "outQueueSize") this->outQueueSize = stoi(param);
        if(obs == "useSignatures") this->useSignatures = stoi(param);
    }
};

class totalordermulticast : public systema{
    public: 
    
    string fairbatch = "true";
    string verifyTimestamps = "false";
    string state_transfer = "true";
    string log = "true";
    string log_parallel = "false";
    string log_to_disk = "false";
    string sync_log = "false";
    string checkpoint_to_disk = "false";
    string sync_ckp = "false";
    long long batchtimeout = -1;
    long long nonces = 10;
    long long highMark = 10000;
    long long revival_highMark = 10;
    long long timeout_highMark = 200;
    long long checkpoint_period = 1024;
    long long global_checkpoint_period = 120000;
    long long maxBatchSizeInBytes = 999999999;
    long long maxbatchsize = 1000;
    long long timeout = 2000;

    void print_sc_name(){
        print_c_name(), mf << "totalordermulticast.";
    }

    void print(){
        print_sc_name(), mf << "batchtimeout = " << batchtimeout << '\n';
        print_sc_name(), mf << "fairbatch = " << fairbatch << '\n';
        print_sc_name(), mf << "nonces = " << nonces << '\n';
        print_sc_name(), mf << "verifyTimestamps = " << verifyTimestamps << '\n';
        print_sc_name(), mf << "state_transfer = " << state_transfer << '\n';
        print_sc_name(), mf << "highMark = " << highMark << '\n';
        print_sc_name(), mf << "revival_highMark = " << revival_highMark << '\n';
        print_sc_name(), mf << "timeout_highMark = " << timeout_highMark << '\n';
        print_sc_name(), mf << "log = " << log << '\n';
        print_sc_name(), mf << "log_parallel = " << log_parallel << '\n';
        print_sc_name(), mf << "log_to_disk = " << log_to_disk << '\n';
        print_sc_name(), mf << "sync_log = " << sync_log << '\n';
        print_sc_name(), mf << "checkpoint_period = " << checkpoint_period << '\n';
        print_sc_name(), mf << "global_checkpoint_period = " << global_checkpoint_period << '\n';
        print_sc_name(), mf << "checkpoint_to_disk = " << checkpoint_to_disk << '\n';
        print_sc_name(), mf << "sync_ckp = " << sync_ckp << '\n';
        print_sc_name(), mf << "maxBatchSizeInBytes = " << maxBatchSizeInBytes << '\n';
        print_sc_name(), mf << "maxbatchsize = " << maxbatchsize << '\n';
        print_sc_name(), mf << "timeout = " << timeout << '\n';
    }

     void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "fairbatch") this->fairbatch = param; 
        if(obs == "verifyTimestamps") this->verifyTimestamps = param; 
        if(obs == "state_transfer") this->state_transfer = param; 
        if(obs == "log") this->log = param; 
        if(obs == "log_parallel") this->log_parallel = param; 
        if(obs == "log_to_disk") this->log_to_disk = param; 
        if(obs == "sync_log") this->sync_log = param; 
        if(obs == "checkpoint_to_disk") this->checkpoint_to_disk = param; 
        if(obs == "sync_ckp") this->sync_ckp = param; 
        if(obs == "batchtimeout") this->batchtimeout = stoi(param);
        if(obs == "nonces") this->nonces = stoi(param);
        if(obs == "highMark") this->highMark = stoi(param);
        if(obs == "revival_highMark") this->revival_highMark = stoi(param);
        if(obs == "timeout_highMark") this->timeout_highMark = stoi(param);
        if(obs == "checkpoint_period") this->checkpoint_period = stoi(param);
        if(obs == "global_checkpoint_period") this->global_checkpoint_period = stoi(param);
        if(obs == "maxBatchSizeInBytes") this->maxBatchSizeInBytes = stoi(param);
        if(obs == "maxbatchsize") this->maxbatchsize = stoi(param);
        if(obs == "timeout") this->timeout = stoi(param);

    }
};

class ssltls : public systema{
    public:

    string protocol_version = "TLSv1.2";
    string key_store_file = "EC_KeyPair_256.pkcs12";
    string enabled_ciphers = "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256";

    void print_sc_name(){
        print_c_name(); mf << "ssltls.";
    }

    void print(){
        print_sc_name(), mf << "protocol_version = " << protocol_version << '\n';
        print_sc_name(), mf << "key_store_file = " << key_store_file << '\n';
        print_sc_name(), mf << "enabled_ciphers = " << enabled_ciphers << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "protocol_version") this->protocol_version = param;
        if(obs == "key_store_file") this->key_store_file = param;
        if(obs == "enabled_ciphers") this->enabled_ciphers = param;
    }

};

class ttp : public systema{
    public: 

    long long id = 7002;

    void print_sc_name(){
        print_c_name(); mf << "ttp.";
    }

    void print(){
        print_sc_name(), mf << "id = " << id << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "id") this->id = stoi(param);
    }
};

class initial : public systema{
    public:
    
    string view = "0,1,2,3";

    void print_sc_name(){
        print_c_name(); mf << "initial.";
    }

    void print(){
        print_sc_name(), mf << "view = " << view << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "view") this->view = param;
    }
};

class servers : public systema{
    public:

    long long num = 4;
    long long f = 0;

    void print_sc_name(){
        print_c_name(); mf << "servers.";
    }

    void print(){
        print_sc_name(), mf << "num = " << num << '\n';
        print_sc_name(), mf << "f = " << f << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "num") this->num = stoi(param);
        if(obs == "f") this->f = stoi(param);
    }
};

class client : public systema{
    public: 
    
    long long invokeOrderedTimeout = 40;

    void print_sc_name(){
        print_c_name(); mf << "client.";
    }

    void print(){
        print_sc_name(), mf << "invokeOrderedTimeout = " << invokeOrderedTimeout << '\n';
    }

    void update(string obs, string param){
        obs.pop_back(); // Remove the colon
        if(obs == "invokeOrderedTimeout") this->invokeOrderedTimeout = stoi(param);
    }
};


void parse_yaml(systema &ms, communication &com, totalordermulticast &tom, ssltls &tls, ttp &tp, initial &init, servers &ms_servers, client &ms_clients){
    ifstream in_mf;

    in_mf.open("general_config.yaml");

    string cur_observer;
    string param;

    while(in_mf >> cur_observer){
        if(cur_observer == "#"){
            getline(in_mf, cur_observer); // ignore comments
        }
        else{
            if(cur_observer == "COMMUNICATION"){
                in_mf >> cur_observer;
                in_mf >> param;
                com.update(cur_observer, param);
            }
            else if(cur_observer == "TOTAL_ORDER_MULTICAST"){
                in_mf >> cur_observer;
                in_mf >> param;
                tom.update(cur_observer, param);
            }
            else if(cur_observer == "SSLTLS"){
                in_mf >> cur_observer;
                in_mf >> param;
                tls.update(cur_observer, param);
            }
            else if(cur_observer == "TTP"){
                in_mf >> cur_observer;
                in_mf >> param;
                tp.update(cur_observer, param);
            }
            else if(cur_observer == "SYSTEM"){
                in_mf >> cur_observer;
                in_mf >> param;
                ms.update(cur_observer, param);
            }
            else if(cur_observer == "INITIAL"){
                in_mf >> cur_observer;
                in_mf >> param;
                init.update(cur_observer, param);
            }
            else if(cur_observer == "SERVERS"){
                in_mf >> cur_observer;
                in_mf >> param;
                ms_servers.update(cur_observer, param);
            }
            else if(cur_observer == "CLIENT"){
                in_mf >> cur_observer;
                in_mf >> param;
                ms_clients.update(cur_observer, param);
            }
        }
    }

    in_mf.close();
}




signed main(){
    systema ms;
    communication ms_comm;
    totalordermulticast ms_tom;
    ssltls ms_tls;
    ttp ms_ttp;
    initial ms_init;
    servers ms_servers;
    client ms_clients;

    parse_yaml(ms, ms_comm, ms_tom, ms_tls, ms_ttp, ms_init, ms_servers, ms_clients);


    mf.open(".syst_conf");

    ms_comm.print(), mf << '\n';
    ms_tom.print(), mf << '\n';
    ms_tls.print(), mf << '\n';
    ms_ttp.print(), mf << '\n';
    ms.print(), mf << '\n';
    ms_init.print(), mf << '\n';
    ms_servers.print(), mf << '\n';
    ms_clients.print(), mf << '\n';

    mf.close();
}