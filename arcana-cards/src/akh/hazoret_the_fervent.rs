//! Hazoret the Fervent — `{3}{R}` 5/4 Legendary God.
//! Indestructible, haste. "Hazoret can't attack or block unless you have one
//! or fewer cards in hand." (static — GAP)
//! "{2}{R}, Discard a card: Hazoret deals 2 damage to each opponent."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hazoret the Fervent");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Indestructible, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static "can't attack or block unless you have one or fewer cards in hand" — no conditional attack/block restriction primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, Discard a card: Hazoret deals 2 damage to each opponent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_each_opponent,
            }),
    )
}

fn damage_each_opponent(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 2,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
