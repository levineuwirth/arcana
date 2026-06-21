//! Lonis, Cryptozoologist — `{G}{U}` 1/2 Legendary Snake Elf Scout.
//!
//! * Whenever another nontoken creature you control enters, investigate
//!   (create a Clue token).
//! * `{T}, Sacrifice X Clues: Target opponent reveals the top X cards…`
//!   — a variable-X sacrifice cost is not expressible (GAP).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lonis, Cryptozoologist");
    let snake = reg.interner_mut().intern("Snake");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP (activated): "{T}, Sacrifice X Clues: …" — a variable-X choose-how-
    // many sacrifice cost is not expressible in ActivationCost.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "another" self-exclusion not a documented filter refinement.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
