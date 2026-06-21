//! Jace's Mindseeker — `{4}{U}{U}` 4/4 Fish Illusion with Flying.
//! "When this creature enters, target opponent mills five cards. You may
//! cast an instant or sorcery spell from among them without paying its
//! mana cost."
//!
//! Flying is a base keyword ("Mill" is a Scryfall ability keyword, not a
//! KeywordAbility variant — it is the ETB). The ETB mills the targeted
//! player five cards. The "you may cast an instant/sorcery from among
//! them for free" rider has no primitive (ImpulseExile only enables
//! casting from your OWN exiled library cards, not an opponent's
//! graveyard), so that clause is GAP'd; the targeted mill is wired.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Jace's Mindseeker");
    let fish = reg.interner_mut().intern("Fish");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_mill_five,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn etb_mill_five(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: "You may cast an instant or sorcery spell from among them without
    // paying its mana cost." — no primitive enables casting from an opponent's
    // milled cards for free (ImpulseExile only flags your OWN library cards).
    vec![Effect::Mill {
        player: *p,
        count: 5,
    }]
}
