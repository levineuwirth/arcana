//! Aryel, Knight of Windgrace — `{2}{W}{B}` 4/4 Legendary Human Knight.
//! Vigilance.
//! {2}{W}, {T}: Create a 2/2 white Knight creature token with vigilance.
//! {B}, {T}, Tap X untapped Knights you control: Destroy target creature with
//! power X or less.
//!
//! Abilities:
//!  - Vigilance → `KeywordAbility::Vigilance`.
//!  - Activated {2}{W}, {T}: create a 2/2 white Knight with vigilance.
//!  - Activated {B}, {T}, Tap X untapped Knights: destroy target creature with
//!    power X or less. The X-variable tap cost (X is chosen, and the destroy's
//!    power restriction is bound to that same X) is not expressible: the
//!    `tap_other_count` field is a FIXED count and there is no way to relate a
//!    target's power bound to the number of permanents tapped → this ability
//!    is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "{B}, {T}, Tap X untapped Knights you control: Destroy target creature
// with power X or less." — the variable-X tap cost and the X-bound target power
// restriction are not expressible (tap_other_count is a fixed integer; no
// dynamic-X tap-cost / power-bound linkage primitive exists).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aryel, Knight of Windgrace");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    // Pre-intern the token subtype so the resolver can rebuild it.
    let _token_knight = reg.interner_mut().intern("Knight");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}, {T}: Create a 2/2 white Knight creature token with vigilance."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_knight_token,
        }),
    )
}

fn make_knight_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
