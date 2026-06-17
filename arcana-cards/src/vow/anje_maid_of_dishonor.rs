//! Anje, Maid of Dishonor — `{2}{B}{R}` 4/5 Legendary Vampire (B/R).
//! Whenever Anje and/or one or more other Vampires you control enter, create a Blood
//! token (once each turn). {2}, Sacrifice another creature: each opponent loses 2 life
//! and you gain 2 life (the "or a Blood token" sacrifice alternative is a fidelity GAP).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anje, Maid of Dishonor");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let vampire_filter = ObjectFilter::new()
        .with_subtype_sym(vampire)
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: vampire_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_blood,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice another creature or a Blood token: Each opponent loses 2 life and you gain 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    // GAP: "or a Blood token" — sacrifice_other is one ObjectFilter;
                    // restricted to another creature you control.
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: drain_each_opponent,
            }),
    )
}

fn make_blood(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Blood,
        count: 1,
    }]
}

fn drain_each_opponent(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: ctx.controller, amount: 2 });
    effects
}
