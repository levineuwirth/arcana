//! Clear the Stage — `{4}{B}` instant. "Target creature gets -3/-3
//! until end of turn. If you control a creature with power 4 or
//! greater, you may return up to one target creature card from your
//! graveyard to your hand."

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clear the Stage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -3/-3 until end of turn. If you control a creature with power 4 or greater, you may return up to one target creature card from your graveyard to your hand.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::UpTo(1),
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
    let mut effects = Vec::new();
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = first else { return Vec::new(); };
    effects.push(Effect::Pump {
        target: *id,
        power: -3,
        toughness: -3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    });
    let has_big = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    ) > 0;
    if has_big {
        if let Some(TargetChoice::Object(gy_id)) = entry.targets.targets.get(1) {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *gy_id });
        }
    }
    effects
}
