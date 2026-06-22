//! Kessig Wolfrider — `{R}` 1/2 Creature — Human Knight.
//! Menace.
//! {2}{R}, {T}, Exile three cards from your graveyard: Create a 3/2 red Wolf
//! creature token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kessig Wolfrider");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    // Pre-intern the Wolf token subtype for the resolver.
    let _wolf = reg.interner_mut().intern("Wolf");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, {T}, Exile three cards from your graveyard: Create a 3/2 red Wolf creature token.".into(),
                // GAP: "Exile three cards from your graveyard" is not an
                // expressible ActivationCost field; only the {2}{R} + {T}
                // portion of the cost is modeled.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_wolf,
            }),
    )
}

fn make_wolf(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg
        .interner()
        .lookup("Wolf")
        .expect("Wolf interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: wolf,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
