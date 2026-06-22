//! Draconic Disciple — `{1}{R}{G}` 2/2 red/green Human Shaman.
//!
//! * `{T}: Add one mana of any color.`  (GAP'd — see below.)
//! * `{7}, {T}, Sacrifice this creature: Create a 5/5 red Dragon creature
//!   token with flying.`
//!
//! The any-color mana ability is GAP'd: `Effect::AddMana` only mints a
//! specific `ManaColor` pip (`ManaUnit::plain`); there is no any-color /
//! player-chosen-color mana primitive in scope (matches Ceta Disciple). The
//! sacrifice activation that mints the Dragon token is expressed faithfully.

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
    let name = reg.interner_mut().intern("Draconic Disciple");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    // Pre-intern the token's subtype so the resolver can look it up.
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "{T}: Add one mana of any color." No any-color mana primitive is in
    // scope; AddMana mints only a specific ManaColor pip.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{7}, {T}, Sacrifice this creature: Create a 5/5 red Dragon creature token with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{7}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_dragon,
            }),
    )
}

fn make_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
