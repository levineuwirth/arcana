//! Chamber Sentry — `{X}` 0/0 Artifact Creature — Construct.
//! "This creature enters with a +1/+1 counter on it for each color of mana spent to cast it." (GAP — colors-spent ETB counters)
//! "{X}, {T}, Remove X +1/+1 counters from this creature: It deals X damage to any target." (GAP — X-scaled +1/+1 counter removal not expressible)
//! "{W}{U}{B}{R}{G}: Return this card from your graveyard to your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chamber Sentry");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with a +1/+1 counter for each color of mana spent to cast it" — not expressible.
    // GAP: "{X}, {T}, Remove X +1/+1 counters: deals X damage to any target" — X-scaled
    //      +1/+1-counter removal is not in the activation-cost surface.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}{U}{B}{R}{G}: Return this card from your graveyard to your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_from_graveyard,
        }),
    )
}

fn return_from_graveyard(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
