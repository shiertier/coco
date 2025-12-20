# 服务层级

不同的服务层级允许您根据应用程序的需求在可用性、性能和可预测成本之间取得平衡。

---

我们提供三个服务层级：
- **Priority Tier（优先级层级）：** 最适合在生产环境中部署的工作流，其中时间、可用性和可预测的定价很重要
- **Standard（标准）：** 用于试点和扩展日常用例的默认层级
- **Batch（批处理）：** 最适合可以等待或受益于在正常容量之外运行的异步工作流

## Standard Tier（标准层级）

标准层级是所有 API 请求的默认服务层级。此层级中的请求与所有其他请求一起被优先处理，并遵循尽力而为的可用性。

## Priority Tier（优先级层级）

此层级中的请求在 Anthropic 的所有其他请求中被优先处理。此优先级处理有助于最小化 ["server overloaded" 错误](/docs/zh-CN/api/errors#http-errors)，即使在高峰期也是如此。

有关更多信息，请参阅 [Priority Tier 入门](#get-started-with-priority-tier)

## 请求如何被分配层级

在处理请求时，Anthropic 在以下情况下决定将请求分配给 Priority Tier：
- 您的组织具有足够的优先级层级容量 **input** 令牌/分钟
- 您的组织具有足够的优先级层级容量 **output** 令牌/分钟

Anthropic 按如下方式计算针对 Priority Tier 容量的使用情况：

**Input Tokens（输入令牌）**
- 缓存读取为从缓存读取的每个令牌 0.1 个令牌
- 缓存写入为写入缓存的每个令牌 1.25 个令牌，TTL 为 5 分钟
- 缓存写入为写入缓存的每个令牌 2.00 个令牌，TTL 为 1 小时
- 对于 [long-context](/docs/zh-CN/build-with-claude/context-windows)（>200k 输入令牌）请求，输入令牌为每个令牌 2 个令牌
- 所有其他输入令牌为每个令牌 1 个令牌

**Output Tokens（输出令牌）**
- 对于 [long-context](/docs/zh-CN/build-with-claude/context-windows)（>200k 输入令牌）请求，输出令牌为每个令牌 1.5 个令牌
- 所有其他输出令牌为每个令牌 1 个令牌

否则，请求以标准层级进行。

<Note>
分配给 Priority Tier 的请求从 Priority Tier 容量和常规速率限制中提取。
如果处理请求会超过速率限制，则请求被拒绝。
</Note>

## 使用服务层级

您可以通过设置 `service_tier` 参数来控制哪些服务层级可用于请求：

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # 在可用时自动使用 Priority Tier，否则回退到标准层级
)
```

`service_tier` 参数接受以下值：

- `"auto"`（默认）- 如果可用，使用 Priority Tier 容量，否则回退到您的其他容量
- `"standard_only"` - 仅使用标准层级容量，如果您不想使用 Priority Tier 容量，这很有用

响应 `usage` 对象还包括分配给请求的服务层级：

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
这允许您确定哪个服务层级被分配给了请求。

当使用具有 Priority Tier 承诺的模型请求 `service_tier="auto"` 时，这些响应标头提供见解：
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
您可以使用这些标头的存在来检测您的请求是否符合 Priority Tier 的条件，即使它超过了限制。

## Priority Tier 入门

如果您对以下内容感兴趣，您可能想要承诺 Priority Tier 容量：
- **更高的可用性**：目标 99.5% 的正常运行时间，具有优先级计算资源
- **成本控制**：可预测的支出和更长承诺期的折扣
- **灵活溢出**：当您超过承诺容量时自动回退到标准层级

承诺 Priority Tier 将涉及决定：
- 每分钟输入令牌数
- 每分钟输出令牌数
- 承诺期限（1、3、6 或 12 个月）
- 特定的模型版本

<Note>
您购买的输入令牌与输出令牌的比率很重要。调整您的 Priority Tier 容量以与您的实际流量模式相符有助于您最大化购买令牌的利用率。
</Note>

### 支持的模型

Priority Tier 支持：

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7（[已弃用](/docs/zh-CN/about-claude/model-deprecations)）
- Claude Haiku 3.5（[已弃用](/docs/zh-CN/about-claude/model-deprecations)）

查看 [模型概览页面](/docs/zh-CN/about-claude/models/overview) 了解有关我们模型的更多详情。

### 如何访问 Priority Tier

要开始使用 Priority Tier：

1. [联系销售](https://claude.com/contact-sales/priority-tier) 完成配置
2. （可选）更新您的 API 请求以可选地将 `service_tier` 参数设置为 `auto`
3. 通过响应标头和 Claude 控制台监控您的使用情况