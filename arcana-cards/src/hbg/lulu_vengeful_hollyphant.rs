//! Lulu, Vengeful Hollyphant — `{2}{W}{B}` 2/4 Legendary Elephant Angel.
//! Flying.
//! Whenever you attack with one or more other creatures with flying, each
//! opponent loses that much life and you gain that much life.
//!
//! The keyword line (Flying) is a base characteristic. The attack trigger is
//! a once-per-combat batch trigger ("attack with one or more …") whose amount
//! ("that much") is the count of OTHER attacking creatures with flying — there
//! is no single TriggerCondition for a batched attack declaration, and no
//! PendingTrigger accessor exposing the full attacking set to derive the
//! count, so the effect is GAP'd (a per-creature CreatureAttacks fire would be
//! materially wrong: it would multi-fire and lose the batch count).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lulu, Vengeful Hollyphant");
    let elephant = reg.interner_mut().intern("Elephant");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Closest available trigger; Lulu itself attacking is a proxy for
            // the attack declaration. See GAP below for why the effect is empty.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: gap_attack_with_flyers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gap_attack_with_flyers(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attack with one or more other creatures with flying → each opponent
    // loses that much life and you gain that much life." The amount is the
    // count of OTHER attacking flyers, which requires reading the full set of
    // declared attackers from the combat state — no script/accessor exposes it,
    // and there is no batched-attack TriggerCondition.
    Vec::new()
}
