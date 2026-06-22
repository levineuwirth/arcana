//! Advanced Stitchwing — `{3}{U}{U}` 3/4 Zombie Horror with Flying.
//! "{2}{U}, Discard two cards: Return this card from your graveyard to the
//!  battlefield tapped."
//!
//! Flying is a base keyword. The graveyard-activated recursion is wired with a
//! two-card discard cost; the "tapped" rider on the return is not expressible
//! (ReturnFromGraveyardToBattlefield has no tapped flag — GAP fidelity).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Advanced Stitchwing");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}, Discard two cards: Return this card from your graveyard to the battlefield tapped.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                discard_other: Some(ObjectFilter::default()),
                discard_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_from_gy,
        }),
    )
}

fn return_self_from_gy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "tapped" — no tapped flag on ReturnFromGraveyardToBattlefield.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
