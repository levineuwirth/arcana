//! Collector Protector — `{3}{W}{W}` 2/5 white Creature — Human Gamer.
//! {W}, Give an opponent a nonland card you own from outside the game: Prevent the next 1 damage that would be dealt to you or this creature this turn.
//! GAP: "give an opponent a card from outside the game" — cost not in ActivationCost catalog.
//! GAP: "prevent next 1 damage to you or this creature" — targeted damage prevention requiring choice not in catalog.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Collector Protector");
    let human_sub = reg.interner_mut().intern("Human");
    let gamer_sub = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(gamer_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, give an opponent a nonland card from outside the game: Prevent the next 1 damage to you or this creature this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: prevent_damage,
            }),
    )
}

fn prevent_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "give an opponent a card from outside the game" — additional cost not in catalog
    // GAP: "prevent next 1 damage to you or this creature" — choice of target not in catalog
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
