//! Bleeding Edge — `{1}{B}{B}` sorcery. "Up to one target creature gets
//! -2/-2 until end of turn. Amass Zombies 2."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bleeding Edge");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Up to one target creature gets -2/-2 until end of turn. Amass Zombies 2.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Amass Zombies 2" (grow/create an Army token) has no
    // primitive; emitting the -2/-2 half only.
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::Pump {
            target: *id,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
