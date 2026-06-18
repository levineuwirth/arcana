//! Sengir Nosferatu — `{3}{B}{B}` 4/4 Vampire with Flying.
//! `{1}{B}, Exile this creature: Create a 1/2 black Bat creature token with
//! flying. It has "{1}{B}, Sacrifice this token: Return an exiled card named
//! Sengir Nosferatu to the battlefield under its owner's control."`
//!
//! Flying is a base keyword. The activated ability ({1}{B} + exile self)
//! creates a 1/2 black Bat with flying — its bones are faithful. The token's
//! granted activated ability (return the exiled Sengir Nosferatu) cannot be
//! authored on a TokenDefinition with the available API and is a GAP.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sengir Nosferatu");
    let vampire = reg.interner_mut().intern("Vampire");
    // Pre-intern the Bat token subtype so the resolver can look it up.
    let _bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, Exile this creature: Create a 1/2 black Bat creature token with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_bat,
            }),
    )
}

// GAP: the created Bat's granted activated ability ("{1}{B}, Sacrifice this
// token: Return an exiled card named Sengir Nosferatu to the battlefield")
// cannot be authored on a TokenDefinition with the available API.
fn make_bat(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bat = reg.interner().lookup("Bat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: bat,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
