//! Leering Gargoyle — `{1}{W}{U}` 2/2 Gargoyle with Flying.
//! "{T}: This creature gets -2/+2 and loses flying until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leering Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature gets -2/+2 and loses flying until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_plus_two,
            }),
    )
}

fn minus_two_plus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses flying until end of turn" — no single-keyword removal effect;
    // only the -2/+2 is expressible.
    vec![Effect::Pump {
        target: ctx.source,
        power: -2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
