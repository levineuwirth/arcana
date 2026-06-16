//! Kasmina, Enigmatic Mentor — `{3}{U}` legendary planeswalker, starting
//! loyalty 5.
//!
//! Oracle:
//! * (static) Spells your opponents cast that target a creature or
//!   planeswalker you control cost `{2}` more to cast.
//! * `−2`: Create a 2/2 blue Wizard creature token. Draw a card, then
//!   discard a card.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller
//!   only, once per turn per planeswalker.
//! * CR 704.5i — 0 loyalty → graveyard SBA.
//!
//! # Scope
//!
//! * The static cost-increase ability is NOT a loyalty ability and a
//!   continuous "spells that target your stuff cost {2} more" tax has
//!   no demonstrated `Effect` surface here — it is omitted (a GAP, not
//!   a loyalty line).
//! * `−2`: fully modeled — mints a 2/2 blue Wizard token, then draws a
//!   card and discards a card.

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kasmina, Enigmatic Mentor");
    let kasmina = reg.interner_mut().intern("Kasmina");
    // Intern the token subtype now so the resolver can `lookup` it.
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kasmina);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "−2: Create a 2/2 blue Wizard creature token. Draw a \
                       card, then discard a card."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_token,
            },
        ),
    )
}

/// `−2: Create a 2/2 blue Wizard token. Draw a card, then discard a
/// card.`
fn minus_two_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wizard = reg.interner().lookup("Wizard").expect("Wizard interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);

    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
