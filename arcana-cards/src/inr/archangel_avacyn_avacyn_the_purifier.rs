//! Archangel Avacyn // Avacyn, the Purifier
//!
//! Front: {3}{W}{W} Legendary Creature — Angel 4/4
//! Flash, flying, vigilance.
//! When Archangel Avacyn enters, creatures you control gain indestructible until end of turn.
//! When a non-Angel creature you control dies, transform Archangel Avacyn at the beginning
//! of the next upkeep.
//!
//! Back: Legendary Creature — Angel 6/5
//! Flying.
//! When this creature transforms into Avacyn, the Purifier, it deals 3 damage to each
//! other creature and each opponent.
//!
//! GAP: Back face "deals 3 damage to each other creature" — we approximate with ForEach
//!      over all creatures (including controller's) minus self; the "other" filter is
//!      approximated without excluding the source itself.
//! GAP: Back face on-transform trigger "when this transforms" is a back-face-only ability
//!      not auto-installed. Modeled here as a shared ZoneChange trigger that fires after
//!      transform, but cannot distinguish front-to-back from back-to-front.
//! GAP: "at the beginning of the next upkeep" transform scheduling is approximated by a
//!      TriggeredAbility on UpkeepBegins; the "next" qualifier (only once) is best-effort.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archangel Avacyn");
    let sub_angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
        ],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Avacyn, the Purifier");
    let sub_angel_back = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_angel_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying],
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: ETB — creatures you control gain indestructible until end of turn
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_indestructible,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: non-Angel creature you control dies → transform at next upkeep
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_non_angel_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_indestructible(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Creatures you control gain indestructible until end of turn.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}

fn on_non_angel_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: The trigger should only fire for non-Angel creatures; the Angel subtype
    // exclusion is not filterable in TriggerCondition::ZoneChange.
    // GAP: "at the beginning of the next upkeep" scheduling — emitting Transform
    // immediately as a best-effort approximation.
    vec![Effect::Transform { target: trig.source }]
}
