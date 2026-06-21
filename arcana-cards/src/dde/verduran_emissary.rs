//! Verduran Emissary — `{2}{G}` 2/3 Human Wizard.
//! "Kicker {1}{R}."
//! "When this creature enters, if it was kicked, destroy target artifact.
//!  It can't be regenerated."
//!
//! Kicker is not in the usable KeywordAbility surface (GAP) and there is no
//! kicker-payment cost field. The ETB destroy is gated on "if it was kicked"
//! — an intervening-if on the kicked state, which has no `conditions::`
//! predicate. Firing the destroy unconditionally would be a materially wrong
//! card, so the whole effect is GAP'd while the targeted trigger shell is
//! retained.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Verduran Emissary");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: Kicker {1}{R} — Kicker is not an available KeywordAbility variant and
    // there is no kicker-payment cost field.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_kicked_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_kicked_destroy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the destroy is gated on "if it was kicked", which is not an
    // expressible intervening-if (no kicked-state condition predicate).
    // Firing unconditionally would be wrong, so the effect is omitted.
    // (Also no "can't be regenerated" rider on DestroyPermanent.)
    Vec::new()
}
