//! Magda, Brazen Outlaw — `{1}{R}` 2/1 Legendary Dwarf Berserker.
//! "Other Dwarves you control get +1/+0." (static — GAP)
//! "Whenever a Dwarf you control becomes tapped, create a Treasure token."
//! "Sacrifice five Treasures: Search your library for an artifact or Dragon
//! card, put that card onto the battlefield, then shuffle."
//!
//! GAP: static — "Other Dwarves you control get +1/+0" (anthem, no trigger/
//! activated form).
//! FIDELITY GAP: the tutor filter expresses "artifact card" only; the "or Dragon
//! card" disjunction (artifact-type OR Dragon-subtype) isn't a single-filter shape.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::effects::CommodityToken;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magda, Brazen Outlaw");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let dwarf_filter =
        script::subtype_filter(reg, "Dwarf").controlled_by(ControllerConstraint::You);
    let treasure_filter = script::subtype_filter(reg, "Treasure");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::BecomesTapped { filter: dwarf_filter },
                intervening_if: None,
                effect: make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice five Treasures: Search your library for an artifact or Dragon card, put that card onto the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(treasure_filter),
                    sacrifice_other_count: 5,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_artifact_to_battlefield,
            }),
    )
}

fn make_treasure(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}

fn tutor_artifact_to_battlefield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: "or Dragon card" half not expressible alongside the artifact
    // type-filter in a single ObjectFilter.
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
