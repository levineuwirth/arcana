//! Ancient Adamantoise — `{5}{G}{G}{G}` 8/20 Turtle with Vigilance and Ward {3}.
//! "Damage isn't removed from this creature during cleanup steps." (static)
//! "All damage that would be dealt to you and other permanents you control is
//! dealt to this creature instead." (static)
//! "When this creature dies, exile it and create ten tapped Treasure tokens."
//!
//! Decomposition:
//! * Vigilance, Ward {3} — keywords.
//! * "Damage isn't removed during cleanup" — static replacement/rules
//!   modification; GAP'd.
//! * "All damage to you/your permanents is dealt to this instead" — static
//!   board-wide redirection replacement; GAP'd (no static-install form here).
//! * Dies trigger — exile this card from the graveyard and create ten Treasure
//!   tokens. ("tapped" on the Treasures is a fidelity gap; commodity Treasure
//!   tokens enter untapped.)

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Ancient Adamantoise");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(20)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: "Damage isn't removed during cleanup" — static rules modification.
    // GAP: "All damage to you/your permanents is dealt to this instead" — static
    //      board-wide damage redirection replacement.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_exile_make_treasures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_exile_make_treasures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dying = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::Sequence(vec![
        Effect::ExileFromGraveyard { target: dying },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 10,
        },
    ])]
}
