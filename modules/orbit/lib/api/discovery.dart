// lib/api/discovery.dart
// Discover Elysia API instances on the local network using mDNS
// Requires the multicast_dns package
// Add to pubspec.yaml: multicast_dns: ^0.3.0
// Usage: ElysiaDiscovery.discover()
// Returns a list of base URLs (e.g., ["http://
//
import 'dart:io';
import 'package:multicast_dns/multicast_dns.dart';

class ElysiaDiscovery {
  static Future<List<String>> discover() async {
    final client = MDnsClient();
    await client.start();

    List<String> servers = [];

    // Zoek PTR records voor onze service
    await for (final PtrResourceRecord ptr in client.lookup<PtrResourceRecord>(
      ResourceRecordQuery.serverPointer('_elysia._tcp.local'),
    )) {

      // Zoek bijbehorende SRV records
      await for (final SrvResourceRecord srv
          in client.lookup<SrvResourceRecord>(
        ResourceRecordQuery.service(ptr.domainName),
      )) {

        final host = srv.target;
        final port = srv.port;

List<InternetAddress> ips;

try {
  ips = await InternetAddress.lookup(host);
} catch (_) {
  // HOSTNAME FAIL? → Android los dit niet op → gebruik host direct als IP
  ips = [InternetAddress(host)];
}
        for (var ip in ips) {
          servers.add("http://${ip.address}:$port");
        }
      }
    }

    client.stop();
    return servers;
  }
}
