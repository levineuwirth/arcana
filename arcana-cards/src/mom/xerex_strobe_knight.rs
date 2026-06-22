//! Xerex Strobe-Knight — `{2}{U}` 2/2 Creature — Human Knight.
//!
//! Oracle:
//! * Flying, vigilance.
//! * {T}: Create a 2/2 white and blue Knight creature token with vigilance.
//!   Activate only if you've cast two or more spells this turn.
//!
//! Decomposition: Flying + Vigilance keywords + one tap-activated ability that
//! mints a 2/2 W/U Knight token with vigilance, gated by an activation
//! condition ("cast two or more spells this turn").

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xerex Strobe-Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Create a 2/2 white and blue Knight creature token with vigilance. \
                   Activate only if you've cast two or more spells this turn."
                .into(),
            cost: ActivationCost {
                tap: true,
                activation_condition: Some(|s, _src, you, _reg| {
                    script::spells_cast_this_turn(s, &ObjectFilter::new(), you) >= 2
                }),
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
    // GAP: the token's printed ability ("This token saddles Mounts and crews
    //      Vehicles as though its power were 2 greater") is not expressible —
    //      minting the bones (2/2 W/U Knight with vigilance).
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
