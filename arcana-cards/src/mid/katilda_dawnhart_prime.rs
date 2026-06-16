//! Katilda, Dawnhart Prime — `{G}{W}` 1/1 Legendary Human Warlock.
//! "Protection from Werewolves. Human creatures you control have '{T}:
//! Add one mana of any of this creature's colors.' {4}{G}{W}, {T}: Put
//! a +1/+1 counter on each creature you control."
//!
//! Protection is not in the usable keyword surface (GAP'd). The
//! "Human creatures you control have '{T}: Add mana'" granted-ability
//! static has no demonstrated primitive and is GAP'd. The
//! counter-on-each-creature activation is expressible via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Katilda, Dawnhart Prime");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Protection from Werewolves — Protection is not in the
        // usable keyword surface.
        ..Default::default()
    };

    // GAP: "Human creatures you control have '{T}: Add one mana of any of
    // this creature's colors'" — granting an activated mana ability to
    // other permanents has no demonstrated primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: counter_each_creature,
        }),
    )
}

fn counter_each_creature(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
