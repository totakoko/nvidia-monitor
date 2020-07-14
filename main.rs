extern crate nvml_wrapper as nvml;
extern crate size;
extern crate systemstat;
use systemstat::Platform;
use nvml::NVML;

fn start_collector_loop() -> Result<(), Box<dyn std::error::Error>> {
  let nvml = NVML::init()?;
  let device = nvml.device_by_index(0)?;
  let sys = systemstat::System::new();

  loop {
    let cpu_load_stats = sys.cpu_load_aggregate()?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    let cpu_load = cpu_load_stats.done()?;
    let cpu_usage = cpu_load.user + cpu_load.system + cpu_load.nice + cpu_load.interrupt;
    let cpu_total = cpu_usage + cpu_load.idle;
    let mem = sys.memory()?;
    let utilization_rates = device.utilization_rates()?;
    let memory_info = device.memory_info()?;

    let cpu_info = format!("CPU) USAGE: {cpu_load}%   MEM: {mem_usage}/{mem_total}   TEMP: {cpu_temp}°C",
      cpu_temp=sys.cpu_temp()?,
      cpu_load=(100. * (cpu_usage/ cpu_total)).round(),
      mem_usage=systemstat::saturating_sub_bytes(mem.total, mem.free),
      mem_total=mem.total
    );

    let gpu_info = format!("GPU) USAGE: {gpu_utilization}% (MEM: {memory_utilization}%)   MEM: {memory_used}/{memory_total} ({memory_percentage}%)   TEMP: {temperature}°C   POWER: {power_usage}/{power_max}W   FAN: {fan_speed}",
      fan_speed=device.fan_speed(0)?,
      temperature=device.temperature(nvml::enum_wrappers::device::TemperatureSensor::Gpu)?,
      power_usage=device.power_usage()?/1000,
      power_max=device.power_management_limit()?/1000,
      gpu_utilization=utilization_rates.gpu,
      memory_utilization=utilization_rates.memory,
      memory_used=size::Size::Bytes(memory_info.used),
      memory_total=size::Size::Bytes(memory_info.total),
      memory_percentage=100 * memory_info.used / memory_info.total,
    );
    println!("{esc}[2J{esc}[1;1H{cpu}\n{gpu}", esc = 27 as char, cpu = cpu_info, gpu = gpu_info);
  }
}

fn main() {
  start_collector_loop().unwrap();
}
