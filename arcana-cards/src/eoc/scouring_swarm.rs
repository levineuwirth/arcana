//! Scouring Swarm — `{1}{B}{G}` 1/1 Creature — Insect.
//! Flying.
//! Whenever you sacrifice a land, create a tapped token that's a copy of
//! this creature if seven or more land cards are in your graveyard.
//! Otherwise, create a tapped 1/1 black Insect creature token with flying.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scouring Swarm");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "Whenever you sacrifice a land"
            trigger_condition: TriggerCondition::Sacrificed {
                filter: ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: on_sac_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_sac_land(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // "if seven or more land cards are in your graveyard" — branch at
    // resolution. graveyard_matching counts the land cards faithfully.
    let land_filter = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let lands_in_yard =
        script::graveyard_matching(state, &land_filter, trig.controller, trig.controller);

    if lands_in_yard >= 7 {
        // GAP (fidelity): "a tapped token" — CopyPermanent mints an
        // untapped copy; the engine has no tapped-copy variant.
        vec![Effect::CopyPermanent { target: trig.source }]
    } else {
        let insect = reg.interner().lookup("Insect").unwrap_or_default();
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(insect);
        // GAP (fidelity): "tapped" — CreateToken mints an untapped token.
        vec![Effect::CreateToken {
            controller: trig.controller,
            token: arcana_core::effects::TokenDefinition {
                name: insect,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }]
    }
}
