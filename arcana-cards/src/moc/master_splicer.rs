//! Master Splicer — `{3}{W}` 1/1 Phyrexian Human Artificer.
//!
//! Oracle:
//! * When this creature enters, create a 3/3 colorless Phyrexian Golem artifact
//!   creature token.
//! * Golems you control get +1/+1. (static anthem — GAP'd below.)
//!
//! The ETB token is expressible (a 3/3 colorless artifact-creature Golem). The
//! "Golems you control get +1/+1" anthem is a pure static continuous ability
//! with no triggered/activated representation in this shape, so it is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master Splicer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let golem = reg.interner_mut().intern("Golem");
    // Token subtype (also Phyrexian Golem).
    let _golem_token = reg.interner_mut().intern("Golem");
    let _phyrexian_token = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let _ = golem;

    // GAP: "Golems you control get +1/+1." — a static anthem with no
    // triggered/activated representation in this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_golem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_golem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let mut token_subs = SubtypeSet::default();
    token_subs.0.insert(phyrexian);
    token_subs.0.insert(golem);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: golem,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subs,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
