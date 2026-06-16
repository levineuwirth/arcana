//! Infected Vermin — `{2}{B}` 1/1 Rat.
//! "{2}{B}: This creature deals 1 damage to each creature and each player."
//! "Threshold — {3}{B}: This creature deals 3 damage to each creature
//!  and each player. Activate only if there are seven or more cards in
//!  your graveyard."
//!
//! Two activated abilities, each dealing N damage to every creature
//! (ForEach) and every player (Sequence of DealDamage). The Threshold
//! ability is gated by an `activation_condition` requiring 7+ cards in
//! your graveyard. The Threshold keyword line is GAP'd (no
//! `KeywordAbility::Threshold`).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infected Vermin");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Threshold keyword line — no KeywordAbility::Threshold.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: This creature deals 1 damage to each creature and each player.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blast_1,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: This creature deals 3 damage to each creature and each player. Activate only if there are seven or more cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    activation_condition: Some(threshold_gate),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blast_3,
            }),
    )
}

fn threshold_gate(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    arcana_core::conditions::graveyard_at_least(s, you, 7)
}

fn blast(state: &GameState, ctx: &ActivationContext, amount: u32) -> Vec<Effect> {
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let mut effects = vec![Effect::ForEach {
        targets: creatures,
        effect: Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount,
        }),
    }];
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount,
        });
    }
    vec![Effect::Sequence(effects)]
}

fn blast_1(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    blast(state, ctx, 1)
}

fn blast_3(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    blast(state, ctx, 3)
}
