//! Deep Forest Hermit — `{3}{G}{G}` 1/1 Elf Druid with Vanishing 3.
//!
//! Oracle:
//! * Vanishing 3 (keyword — engine handles the time-counter enters-with,
//!   per-upkeep removal, and final sacrifice).
//! * When this creature enters, create four 1/1 green Squirrel creature tokens.
//! * Squirrels you control get +1/+1. (static anthem — GAP'd below.)
//!
//! The ETB creates four 1/1 green Squirrel tokens (four repeated CreateToken
//! values). The "Squirrels you control get +1/+1" anthem is a pure static with
//! no triggered/activated representation in this shape, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Deep Forest Hermit");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let _squirrel_token = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let _ = squirrel;

    // GAP: "Squirrels you control get +1/+1." — a static anthem with no
    // triggered/activated representation in this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vanishing(3)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_squirrels,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_squirrels(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let mut token_subs = SubtypeSet::default();
    token_subs.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
