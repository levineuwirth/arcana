//! Ognis, the Dragon's Lash — `{1}{B/R}{R}{R/G}` 3/3 legendary Lizard
//! Warrior with Haste. "Whenever a creature you control with haste
//! attacks, create a tapped Treasure token."

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ognis, the Dragon's Lash");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/R}{R}{R/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "with haste" qualifier — no keyword predicate in the
            //      demonstrated ObjectFilter refinements; fires on any
            //      attacking creature you control (over-fires the restriction).
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: make_treasure,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_treasure(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // FIDELITY GAP: the token should enter tapped; CreateCommodityToken mints
    // it untapped (no tapped-token primitive in the demonstrated API).
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
