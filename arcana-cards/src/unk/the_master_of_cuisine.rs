//! The Master of Cuisine — `{R}{G}{W}` 3/3 Legendary Human Chef.
//! "Whenever this creature attacks, create a Food token."
//! "Whenever you sacrifice a food, choose one —
//!   • Gelato — Target creature gains haste and trample until end of turn.
//!   • Pasta — Scry 1.
//!   • Pizza — Tap target creature."

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Master of Cuisine");
    let human = reg.interner_mut().intern("Human");
    let chef = reg.interner_mut().intern("Chef");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(chef);
    let food_filter = script::subtype_filter(reg, "Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Scry/Gelato/Pizza/Food/Pasta keyword names are not real
        // KeywordAbility variants (custom/joke keywords); none are emitted.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed { filter: food_filter },
                intervening_if: None,
                effect: sacrifice_food_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_make_food(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}

fn sacrifice_food_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: triggered-ability "choose one —" modal dispatch (Gelato / Pasta /
    // Pizza) is not expressible — the modal machinery (ModalSpec) is a
    // spell-ability feature, not available on a TriggeredAbilityDef, and
    // each mode also targets independently.
    Vec::new()
}
