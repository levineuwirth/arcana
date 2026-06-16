//! Citystalker Connoisseur — `{3}{B}` 3/3 Vampire with Deathtouch.
//! "When this creature enters, target opponent discards a nonland card
//! with the greatest mana value among cards in their hand. Create a
//! Blood token."
//!
//! Deathtouch is a usable keyword. The ETB makes the target player
//! discard a card (engine chooses) and mints a Blood token. The
//! "nonland card with the greatest mana value" specificity is a GAP —
//! a single non-filtered discard is emitted.

use arcana_core::effects::{CommodityToken, DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citystalker Connoisseur");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_and_blood,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_discard_and_blood(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "nonland card with the greatest mana value" specificity — a
    // single unfiltered discard is emitted instead.
    let mut effects = Vec::new();
    if let Some(TargetChoice::Player(p)) = trig.targets.targets.first() {
        effects.push(Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects.push(Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Blood,
        count: 1,
    });
    effects
}
