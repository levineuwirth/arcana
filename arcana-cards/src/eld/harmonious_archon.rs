//! Harmonious Archon — `{4}{W}{W}` 4/5 Archon with Flying.
//! "Flying"
//! "Non-Archon creatures have base power and toughness 3/3." (static — GAP)
//! "When this creature enters, create two 1/1 white Human creature tokens."
//!
//! Flying is the keyword line. The board-wide base-P/T static is not
//! expressible (GAP). The ETB token creation is modeled (two Human tokens).

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
    let name = reg.interner_mut().intern("Harmonious Archon");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);
    // Pre-intern the Human token subtype name for use at resolve time.
    let _human = reg.interner_mut().intern("Human");

    // GAP: "Non-Archon creatures have base power and toughness 3/3." — a
    // board-wide base-P/T-setting static with no expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_humans,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_humans(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}
