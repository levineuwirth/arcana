//! Enter the God-Eternals — `{2}{U}{U}{B}` sorcery. "Deals 4 damage
//! to target creature and you gain life equal to the damage dealt.
//! Target player mills four cards. Amass Zombies 4."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enter the God-Eternals");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Enter the God-Eternals deals 4 damage to target creature and you gain life equal to the damage dealt this way. Target player mills four cards. Amass Zombies 4.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_player(),
                ],
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
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        out.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 4,
        });
        out.push(Effect::GainLife { player: entry.controller, amount: 4 });
    }
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.get(1) {
        out.push(Effect::Mill { player: *p, count: 4 });
    }
    // "Amass Zombies 4" is not expressible (no Army/amass mechanic in
    // the catalog); the damage, lifegain, and mill are applied.
    out
}
