//! Vitu-Ghazi Guildmage — `{G}{W}` 2/2 Dryad Shaman.
//! "{4}{G}{W}: Create a 3/3 green Centaur creature token."
//! The "{2}{G}{W}: Populate" ability (copy a creature token you control,
//! player's choice) has no Populate / choose-a-token-to-copy primitive, so it
//! is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vitu-Ghazi Guildmage");
    let dryad = reg.interner_mut().intern("Dryad");
    let shaman = reg.interner_mut().intern("Shaman");
    reg.interner_mut().intern("Centaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "{2}{G}{W}: Populate" — no Populate effect / choose-a-creature-token
    // -to-copy primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}{W}: Create a 3/3 green Centaur creature token.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_centaur,
        }),
    )
}

fn make_centaur(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let centaur = reg.interner().lookup("Centaur").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: centaur,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
