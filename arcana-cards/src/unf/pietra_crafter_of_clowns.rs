//! Pietra, Crafter of Clowns — `{1}{R}{W}` 3/2 Legendary Human Clown Artificer.
//! Haste.
//! Robots you control get +1/+1.
//! {R}{W}, {T}, Tell a joke you haven't told this game to someone outside the
//! game: Create a 1/1 white Clown Robot artifact creature token. If that person
//! laughed, the token gains haste until end of turn.
//!
//! Abilities:
//!  - Haste → `KeywordAbility::Haste`.
//!  - "Robots you control get +1/+1." — pure static anthem, no expressible
//!    primitive in this card class → GAP'd.
//!  - Activated {R}{W}, {T} + "tell a joke …" (an Un-set out-of-game action):
//!    the joke / "if that person laughed" rider is not a modelable cost or
//!    condition. Emit the modelable portion — the {R}{W}, {T} cost creating a
//!    1/1 white Clown Robot artifact creature token. The conditional
//!    "gains haste if they laughed" is GAP'd (no real-world laughter event).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "Robots you control get +1/+1." — static anthem; no expressible static
// pump primitive for this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pietra, Crafter of Clowns");
    let human = reg.interner_mut().intern("Human");
    let clown = reg.interner_mut().intern("Clown");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(clown);
    subtypes.0.insert(artificer);

    // Pre-intern the token's subtypes so the resolver can rebuild them.
    let _robot = reg.interner_mut().intern("Robot");
    let _token_clown = reg.interner_mut().intern("Clown");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}{W}, {T}, Tell a joke you haven't told this game to someone \
                   outside the game: Create a 1/1 white Clown Robot artifact creature \
                   token. If that person laughed, the token gains haste until end of turn."
                .into(),
            // The "tell a joke …" out-of-game cost component is unmodelable;
            // only the {R}{W}, {T} portion of the cost is represented.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_clown_robot,
        }),
    )
}

fn make_clown_robot(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If that person laughed, the token gains haste until end of turn." —
    // depends on a real-world laughter event, not modelable. The base token is
    // created; the conditional haste rider is dropped.
    let robot = reg.interner().lookup("Robot").unwrap_or_default();
    let clown = reg.interner().lookup("Clown").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clown);
    subtypes.0.insert(robot);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: robot,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
