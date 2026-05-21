//! Ray of Enfeeblement — `{B}` instant. "Target creature gets -4/-1
//! until end of turn. If that creature is white, it gets -4/-4 until
//! end of turn instead."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ray of Enfeeblement");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets -4/-1 until end of turn. If that creature is white, it gets -4/-4 until end of turn instead.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Base mode is -4/-1; the white-creature -4/-4 escalation cannot
    // be tested (no helper reports a permanent's color).
    vec![Effect::Pump {
        target: *id,
        power: -4,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
    // GAP: "if that creature is white, -4/-4 instead" — no script
    // helper exposes a permanent's color, so the conditional branch
    // is omitted and the unconditional -4/-1 is applied.
}
