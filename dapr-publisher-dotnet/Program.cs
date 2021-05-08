using System;
using System.Threading;
using Dapr.Client;
using System.Threading.Tasks;

namespace dapr_publisher_dotnet
{
    internal static class Program
    {
        static async Task<int> Main(string[] args)
        {
            Thread.Sleep(TimeSpan.FromSeconds(3));

            var port = Convert.ToInt32(Environment.GetEnvironmentVariable("DAPR_GRPC_PORT"));
            Console.WriteLine($"Run is at {port}");

            using var client = new DaprClientBuilder().Build();

            for (var index = 1; index <= 100; index++)
            {
                await client.PublishEventAsync("pubsub", "A", $"{index} => hello from dotnet!");
            }

            return 1;
        }
    }
}
