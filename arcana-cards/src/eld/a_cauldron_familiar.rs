//! A-Cauldron Familiar — `{B}` 1/1 Creature — Cat.
//!
//! Oracle:
//! * Cauldron Familiar can't block. (static — GAP'd)
//! * When Cauldron Familiar enters, each opponent loses 1 life and you gain 1 life.
//! * Sacrifice a Food: Return Cauldron Familiar from your graveyard to the battlefield.
//!
//! The "can't block" line is a pure static restriction with no
//! corresponding keyword or static-effect primitive in the catalog, so it
//! is GAP'd. The ETB drain/gain and the graveyard recursion activation
//! are both expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Cauldron Familiar");
    let cat = reg.interner_mut().intern("Cat");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    // "Sacrifice a Food" — chosen Food permanent you control.
    let food_filter = ObjectFilter::new().with_subtype_sym(food);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "Cauldron Familiar can't block" — no can't-block keyword/static primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Food: Return Cauldron Familiar from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(food_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

fn etb_drain(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: 1 });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: 1 });
    effects
}

fn return_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
