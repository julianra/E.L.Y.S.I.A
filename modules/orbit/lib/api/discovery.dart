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

    final List<String> servers = [];

    // PTR lookup
    await for (final PtrResourceRecord ptr in client.lookup<PtrResourceRecord>(
      ResourceRecordQuery.serverPointer('_elysia._tcp.local'),
    )) {
      // SRV lookup for this PTR
      await for (final SrvResourceRecord srv
          in client.lookup<SrvResourceRecord>(
        ResourceRecordQuery.service(ptr.domainName),
      )) {
        final host = srv.target;
        final port = srv.port;

        // Resolve host to IP
        final ips = await InternetAddress.lookup(host);
        for (var ip in ips) {
          servers.add("http://${ip.address}:$port");
        }
      }
    }

    client.stop();
    return servers;
  }
}
