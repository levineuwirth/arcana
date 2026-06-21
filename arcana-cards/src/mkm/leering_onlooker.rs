//! Leering Onlooker — `{1}{B}` 1/3 Vampire with Flying.
//!
//! Oracle:
//! * Flying
//! * "{2}{B}{B}, Exile this card from your graveyard: Create two tapped 1/1
//!   black Bat creature tokens with flying."
//!
//! Partial: the graveyard-activated ability mints two 1/1 black flying Bat
//! tokens. The "tapped" rider on the tokens has no expressible primitive
//! (`CreateToken` enters untapped) — GAP'd.

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
    let name = reg.interner_mut().intern("Leering Onlooker");
    let vampire = reg.interner_mut().intern("Vampire");
    let _bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{B}, Exile this card from your graveyard: Create two tapped 1/1 black Bat creature tokens with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{B}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: make_bats,
            }),
    )
}

fn make_bats(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bat = reg.interner().lookup("Bat").unwrap_or_default();
    let make = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(bat);
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: bat,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }
    };
    // GAP: tokens should enter tapped; CreateToken enters untapped.
    vec![make(), make()]
}
