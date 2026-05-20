//! Trick Shot — `{4}{R}` instant. "Trick Shot deals 6 damage to
//! target creature and 2 damage to up to one other target creature
//! token."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trick Shot");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Trick Shot deals 6 damage to target creature and 2 damage to up to one other target creature token.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().tokens_only(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    let mut it = entry.targets.targets.iter();
    if let Some(TargetChoice::Object(a)) = it.next() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*a),
            amount: 6,
        });
    } else {
        return Vec::new();
    }
    if let Some(TargetChoice::Object(b)) = it.next() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*b),
            amount: 2,
        });
    }
    effects
}
