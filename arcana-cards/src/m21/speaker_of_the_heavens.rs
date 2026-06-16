//! Speaker of the Heavens — `{W}` 1/1 Human Cleric with Vigilance and Lifelink.
//! "{T}: Create a 4/4 white Angel creature token with flying. Activate only
//!  if you have at least 7 life more than your starting life total and only
//!  as a sorcery."
//!
//! GAP (activation gate): the "at least 7 life more than your starting life
//! total" precondition has no documented conditions:: helper (no
//! starting-life-relative predicate), so the gate is omitted. The
//! sorcery-speed restriction is honored via is_instant_speed: false.

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
    let name = reg.interner_mut().intern("Speaker of the Heavens");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let _angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Create a 4/4 white Angel creature token with flying. Activate only if you have at least 7 life more than your starting life total and only as a sorcery.".into(),
            cost: ActivationCost { tap: true, ..ActivationCost::default() },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_angel,
        }),
    )
}

fn make_angel(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: angel,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
