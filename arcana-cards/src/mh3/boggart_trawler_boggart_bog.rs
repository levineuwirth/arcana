//! Boggart Trawler // Boggart Bog — `{2}{B}` Creature — Goblin 3/1 (MDFC).
//!
//! Front face: When this creature enters, exile target player's graveyard.
//!
//! Back face (Land — Boggart Bog):
//! As this land enters, you may pay 3 life. If you don't, it enters tapped.
//! {T}: Add {B}.
//!
//! GAP: "As this land enters, you may pay 3 life. If you don't, it enters tapped." —
//! the ETB conditional-tapped replacement effect is not expressible in the engine.
//!
//! GAP: ETB trigger — "exile target player's graveyard" — bulk ExileGraveyard(PlayerId)
//! effect not in catalog; returning Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boggart Trawler");
    let goblin_sub = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face — Boggart Bog (Land)
    let back_name = reg.interner_mut().intern("Boggart Bog");
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::colorless(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back_face)
            // ETB: exile target player's graveyard
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: etb_exile_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    }
                ],
            }),
    )
}

fn etb_exile_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(_p) = target else { return Vec::new(); };
    // GAP: "Exile target player's graveyard" — no bulk ExileGraveyardOfPlayer effect in catalog.
    Vec::new()
}
