//! Voldaren Ambusher — `{2}{R}` 2/2 red Creature — Vampire Archer.
//! "When this creature enters, if an opponent lost life this turn, it deals X
//! damage to up to one target creature or planeswalker, where X is the number
//! of Vampires you control."
//!
//! GAP: "if an opponent lost life this turn" intervening-if not expressible;
//! planeswalker target not available; X = count of Vampires you control.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voldaren Ambusher");
    let _vampire = reg.interner_mut().intern("Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(archer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if an opponent lost life this turn" not expressible
                intervening_if: None,
                effect: etb_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_deal_damage(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let vampire_filter = script::subtype_filter(reg, "Vampire");
    let x = script::count_matching(state, &vampire_filter, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    // GAP: "if an opponent lost life this turn" not enforced
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: x,
        source: trig.source,
    }]
}
