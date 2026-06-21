//! Thoughtsteal Sprites — `{1}{U}` 2/1 Faerie Rogue with Flying.
//!
//! * Flying.
//! * Double team. (Arena-only keyword — GAP)
//! * Whenever you cast a spell during an opponent's turn, draft a card
//!   from Thoughtsteal Sprites's spellbook, then discard a card. This
//!   ability triggers only once each turn. (the spellbook draft and the
//!   "during an opponent's turn" gate are unexpressible — GAP; the
//!   discard half is wired.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thoughtsteal Sprites");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    // GAP: "Double team" is an Arena-only spellbook keyword; not in the
    // KeywordAbility surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draft_then_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn draft_then_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draft a card from ~'s spellbook" is an Arena spellbook
    // mechanic; not modeled. GAP: the "during an opponent's turn" gate
    // has no expressible condition. The "then discard a card" half is
    // wired below.
    vec![Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
