//! Blizzard Brawl — `{G}` snow sorcery. "Choose target creature you
//! control and target creature you don't control. If you control three
//! or more snow permanents, the creature you control gets +1/+0 and
//! gains indestructible until end of turn. Then those creatures fight
//! each other."
//!
//! Two targets: the creature you control (first) and the creature you
//! don't (second). A resolution-time `script::count_matching` checks
//! the snow-permanent count; if 3+, the first creature gets the pump +
//! indestructible via `Effect::Pump`. Then `Effect::Fight` resolves
//! the brawl.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blizzard Brawl");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature you control and target creature you don't control. If you control three or more snow permanents, the creature you control gets +1/+0 and gains indestructible until end of turn. Then those creatures fight each other.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mine = match entry.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let theirs = match entry.targets.targets.get(1) {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };

    let snow_count = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW)),
        entry.controller,
    );

    let mut effects = Vec::new();
    if snow_count >= 3 {
        effects.push(Effect::Pump {
            target: mine,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Indestructible],
        });
    }
    effects.push(Effect::Fight { a: mine, b: theirs });
    effects
}
